using System;
using FluentAssertions;
using LegacyApp;
using LegacyApp.DataAccess;
using LegacyApp.Models;
using LegacyApp.Repositories;
using LegacyApp.Services;
using NSubstitute;
using Xunit;

namespace RefactoringTest.UnitTest;

public class UserServiceTest
{
    private readonly UserService _sut;

    //add values you want to mock
    private readonly IDateTimeProvider _dateTimeProvider = Substitute.For<IDateTimeProvider>();
    private readonly IClientRepository _clientRepository = Substitute.For<IClientRepository>();
    private readonly IUserCreditService _userCreditService = Substitute.For<IUserCreditService>();
    private readonly IUserDataAccess _userDataAccess = Substitute.For<IUserDataAccess>();


    public UserServiceTest()
    {
        _sut = new UserService(_dateTimeProvider, _clientRepository, _userCreditService, _userDataAccess);
    }

    [Fact]
    public void AddUser_ShouldCreateUser_WhenAllParametersValid()

    {
        // Arrange
        const int clientId = 1;
        const string firstName = "Jan";
        const string lastName = "De Niels";
        var dateOfBirth = new DateTime(1989, 3, 9);
        var client = new Client()
        {
            Id = clientId,
            Name = "ImportantClient",
        };
        //dateTime getter always return fixed value
        _dateTimeProvider.DateTimeNow.Returns(new DateTime(2022, 6, 8));
        _clientRepository.GetById(clientId).Returns(client);
        _userCreditService.GetCreditLimit(firstName, lastName, dateOfBirth).Returns(600);

        // Act
        var result = _sut.AddUser(firstName, lastName, "jdn@gmail.com", dateOfBirth, clientId);

        // Assert
        result.Should().BeTrue();
        // check that _userDataAcces received one call with any user arg
        _userDataAccess.Received(1).AddUser(Arg.Any<User>());
    }

    [Theory]
    [InlineData("", "De Neils", "jdn@gmail.com", 1993)]
    [InlineData("Jan", "", "jdn@gmail.com", 1993)]
    
    [InlineData("Jan", "De Niels", "jdn", 1993)]
    [InlineData("Jan", "De Niels", "jdn@gmail.com", 2004)]
    public void AddUser_ShouldNotCreateUser_WhenAllOrSomeParametersAreInvalid(string firstName, string lastName,
        string email, int yearOfBirth)
    {
        //Arrange

        const int clientId = 1;
        var dateOfBirth = new DateTime(yearOfBirth, 3, 9);
        var client = new Client()
        {
            Id = clientId,
            Name = "ImportantClient"
        };
        _dateTimeProvider.DateTimeNow.Returns(new DateTime(2022, 6, 8));
        _clientRepository.GetById(clientId).Returns(client);
        _userCreditService.GetCreditLimit(Arg.Is(firstName), Arg.Is(lastName), Arg.Is(dateOfBirth)).Returns(600);

        //Act
        var result = _sut.AddUser(firstName, lastName, email, dateOfBirth, clientId);


        //Assert

        result.Should().BeFalse();

        _userDataAccess.Received(0);
    }
}